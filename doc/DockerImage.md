 ```
 ▄▄▄▄▄▄  ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄   ▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄   ▄▄▄▄▄▄▄ ▄▄▄ ▄▄▄     ▄▄▄▄▄▄▄ 
█      ██       █       █   █ █ █       █   ▄  █ █       █   █   █   █       █
█  ▄    █   ▄   █       █   █▄█ █    ▄▄▄█  █ █ █ █    ▄▄▄█   █   █   █    ▄▄▄█
█ █ █   █  █ █  █     ▄▄█      ▄█   █▄▄▄█   █▄▄█▄█   █▄▄▄█   █   █   █   █▄▄▄ 
█ █▄█   █  █▄█  █    █  █     █▄█    ▄▄▄█    ▄▄  █    ▄▄▄█   █   █▄▄▄█    ▄▄▄█
█       █       █    █▄▄█    ▄  █   █▄▄▄█   █  █ █   █   █   █       █   █▄▄▄ 
█▄▄▄▄▄▄██▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄▄▄█ █▄█▄▄▄▄▄▄▄█▄▄▄█  █▄█▄▄▄█   █▄▄▄█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█

```                                                                

# Environment variables

The following environment variables can be provided to the docker image to change its behavior

## db_address

The address of the surrealdb used. 

> __Default:__ 127.0.0.1:8000

Internally the `Surreal::new::<Ws>` client connector is used. So refer to the documentation at [surrealdb.com/docs/integration/sdks/rust](https://surrealdb.com/docs/integration/sdks/rust)

## db_username

The user name used to connect to the database

> __Default:__ root

## db_password

The password associated with the user account used to access the database

> __Default:__ root

## db_namespace

As described at [surrealdb.com/docs/introduction/concepts](https://surrealdb.com/docs/introduction/concepts) the namespace for the database context has to be provided here.

> __Default:__ development

## db_name

The name of the database used in the application. 

> __Default:__ ribs