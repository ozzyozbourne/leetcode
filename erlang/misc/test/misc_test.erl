-module(misc_test).
-include_lib("eunit/include/eunit.hrl").

one_test() -> 
    ?assertEqual(120, misc:factorial(5)),
    ?assertEqual(24, misc:factorial(4)),
    ?assertEqual(6, misc:factorial(3)).
