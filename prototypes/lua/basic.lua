target "hello"
    file { match "./src/*.c" }
    include "./include"
