extern crate d6_doodle;

use clap::{clap_app, crate_version};
use d6_doodle::{models::*, schema::*};
use diesel::prelude::*;

fn main() -> anyhow::Result<()> {
    let clap = clap_app!(doodle_cli =>
        (about:"A cli for the doodle database app")
        (version:crate_version!())
        (author:"Matt Stoodley")
        (@subcommand new_user =>
           (@arg name:+required "The name of the new user")
           (@arg password:+required "the new users password")
         )
        (@subcommand view_users =>)
        (@subcommand new_poll =>
           (@arg question:+required "The question")
           (@arg options:+required "The options (Comma separated)")
           (@arg user_id:-u +takes_value "The owner of the poll")
         )
        (@subcommand view_polls =>)
        (@subcommand respond_poll =>
           (@arg user_id:+required "User ID")
           (@arg poll_id:+required "Poll ID")
           (@arg selection:+required "your selection as an int 0..")
         )
        (@subcommand poll_results =>
           (@arg poll_id:+required "Poll ID")
         )

    )
    .get_matches();

    let conn = d6_doodle::create_connection()?;

    if let Some(ref sub) = clap.subcommand_matches("new_user") {
        let user = new_user(
            sub.value_of("name").unwrap(),
            sub.value_of("password").unwrap(),
        );

        let added: User = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)?;
        println!("New user added = {:?}", added);
    }

    if let Some(ref _sub) = clap.subcommand_matches("view_users") {
        let res = users::table.limit(10).load::<User>(&conn)?;
        for v in res {
            println!("{:?}", v);
        }
    }

    if let Some(ref sub) = clap.subcommand_matches("new_poll") {
        let owner: Option<i32> = match sub.value_of("user_id") {
            Some(v) => Some(v.parse()?),
            None => None,
        };
        let question = sub.value_of("question").unwrap();
        let options = sub.value_of("options").unwrap();
        let added: Poll = diesel::insert_into(polls::table)
            .values(&NewPoll {
                owner,
                question,
                options,
            })
            .get_result(&conn)?;

        println!("Added Poll = {:?}", added);
    }

    if let Some(ref _sub) = clap.subcommand_matches("view_polls") {
        let res = polls::table.limit(10).load::<Poll>(&conn)?;
        for v in res {
            println!("{:?}", v);
        }
    }

    if let Some(ref sub) = clap.subcommand_matches("respond_poll") {
        let new_response = Response {
            poll_id: sub.value_of("poll_id").unwrap().parse()?,
            user_id: sub.value_of("user_id").unwrap().parse()?,
            selected: Some(sub.value_of("selection").unwrap().parse()?),
        };
        let added: Response = diesel::insert_into(responses::table)
            .values(&new_response)
            .get_result(&conn)?;
        println!("Added response = {:?}", added);
    }

    if let Some(ref sub) = clap.subcommand_matches("poll_results") {
        let id: i32 = sub.value_of("poll_id").unwrap().parse()?;
        let poll: Poll = polls::table.find(id).first::<Poll>(&conn)?;
        let vals = responses::table
            .inner_join(users::table)
            .inner_join(polls::table)
            .filter(polls::id.eq(id))
            .select((users::id, responses::selected))
            .load::<(i32, Option<i32>)>(&conn)?;
        println!("{}", poll.question);
        let mut scores = Vec::new();
        for _ in poll.options.split(",") {
            scores.push(0);
        }
        for v in vals {
            if let Some(n) = v.1 {
                let n = n as usize;
                if n < scores.len() {
                    scores[n] += 1;
                }
            } else {
                println!("User: {} has not selected anything", v.0);
            }
        }
        for (n, op) in poll.options.split(",").enumerate() {
            println!("  {}) {} = {}", n, op, scores[n]);
        }
    }
    Ok(())
}
