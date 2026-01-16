# list of todos

- [x]  build out user model, persistance layer with TypeOrm (user.service)
- [x] don't store password in plain text. use bcrypt
  - store only hashed password, then compare the stored password to a hashed version of the *incoming* password
- [] use a DTO class to define the shape of the request body (auth.controller)
  - see the validation chap
- [] how tf do i write tests??
- [x] don't expose `jwtConstants` key publicly
  - use secret vaults, environment variables, or config
