#include <stdio.h>
#include "r_lib.h"
#include <string.h>
#include <stdlib.h>

void main(){
    int d_add = add_double(4,5);
    printf("d_add = %d\n",d_add);


    struct CPoint pt;
    pt.x = 5;
    pt.y = 10;
    more_y(&pt);
    printf("pt.x = %d, pt.y = %d\n",pt.x,pt.y);

    char* str = malloc(20);
    strcpy(str,"What are you doing?");
    
    char* other = edit_string(str);

    printf("other string = %s\n",other);

    s_to_space(other);
    printf("other string without 's' = %s\n",other);

    free(str);
    free(other);
}

