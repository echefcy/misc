fn test1<'a, 'b>(some_ref: &'a str, ret_this_ref : &'b str) -> &'a str{
    some_ref
}

fn test2(some_ref: &str) -> &str{
    ();
    some_ref
}

pub fn driver(){

    {
        let mut diff_string1 = String::from("diff test 1");
        let diff_test_ref = test2(&diff_string1);
        let diff_mut_ref = &mut diff_string1;
        println!("{}", diff_test_ref);
        // Note how this block doesn't work because test2 must return some sort of reference to the passed-in parameter
        // (because otherwise it would create an dangling reference). So we know that diff_test_ref must be
        // an immutable reference to diff_string1. Therefore diff_mut_ref is not allowed to exist 

    }

    {
        let mut string1 = String::from("test1");
        let mut string2 = String::from("test2");
        let test_ref = test1(&string1, &string2);
        let mut_ref = &mut string2;
        println!("{}", test_ref);
        // This block works because we know that test_ref must be a reference with the same lifetime as &string1
        // since the lifetime of &string2 passed in as parameter ends on the same line (it's not used anywhere later, 
        // so non-lexical scope cleans it up), mut_ref can be created successfully.
    }
}