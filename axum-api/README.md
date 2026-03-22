# clean code architecture

## entities
- user

## use cases (application)
- encapsulates and implements all use-cases of the system
- orchestrate the flow of data to and from the entities and direct those entities to use business rules

## interface-adapters
- set of adapaters that convert data from the format most convenient for the uses cases and entities, to
the format most convenient for some external agency such as DB or Web

## frameworks
- axum
- postgres


- `src`:
    - `main.rs`
    - `lib.rs`
    - `entity/`
        - `mod.rs`
        - `user.rs`
    - `application/`
        - `use_case` 
        - `repository` (define the interface) 
        - `service` (define the interface)
    - `interface_adapter/`
        - `controller`
        - `presenter`
    - `infrastructure/`
        - `repository`
        - `service`
    - `framework/`
        - `axum/`
            - `handler/`
            - `route/`
            - `app.rs`
            - `config.rs`
            - `setup.rs`
        - `postgres/`
            - `setup.rs`

entity | application | interface-adapters | infra & frameworks 


[OUTER] -> [INNER]
http (/register POST) -> (call a controller) -> use-cases -> entities -> use-cases -> presenters


# TODOS

- [x] create `sqlx` migration up & down
- [x] create entity user
- [x] complete register user flow
- [x] add logging
    - [] try to find a home for the logging on main.rs - i tried extracting out TracingLayer but couldn't due to my limited knowledge on complicated types
- [x] add password_hash
- [] create errors types
- [] fix unused Results
- [] JWT
- [] create unit test and e2e tests
- [] think about the api routes
