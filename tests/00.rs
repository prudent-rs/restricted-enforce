use private::*;
//use private::prelude::*;

// #[must_use]
fn f() {
    #![allow(unused)]

    #[deprecated]
    macro_rules! unused {
        () => {
            let unused = ();
        };
    }
    macro_rules! allowed_unused {
        () => {
            #[allow(unused)]
            let unused = ();
        };
    }

    {
        #![deny(unused)]
        let _ok_to_be_unused = ();

        // fails to compile - OK:
        //
        //let x = ();

        {
            #![allow(unused)]
            unused!();
        }

        // ok
        allowed_unused!();
    }

    def_let!(ident_here = true);

    def_let! { ident_here = true }
    let _ = at_let!(ident_here);

    {
        def_let_direct!(ident_here = true);
        let _ = at_let!(ident_here);
        let _ = ident_here!();
    }

    def_let!(ident_here@::bufo::bufo:u8=0);
    {
        def_let_direct!(ident_here@::bufo::bufo:u8=0);
        let _ = ident_here!();
    }

    def_let!(ident_here);
    at_let!(ident_here) = 1;

    {
        def_let_direct!(ident_here);
        if true {
            at_let!(ident_here) = 1;
        } else {
            ident_here!() = 0;
        }
    }

    def_let!(ident_here @::dufo::dufo);
    at_let!(ident_here @::dufo::dufo) = true;
    /* */
    def_let!(ident_here@::bufo::bufo:bool);
    at_let!(ident_here@::bufo::bufo) = true;
    def_let!(ident_here@::bufo::bufo:bool);

    /* */
    //bufo_bufo_private_ident_here_dimvxevsdmqmbnuhyptltyqdlnafhdbg= 0;
}
