# Api

## Authentication
### Register

#### Request 
```http
POST /auth/register
```
> **Content-Type : application/json** 

#### Expected form

```json
{
    "email" : "testing@gmail.com",
    "first_name" : "fname",
    "last_name" : "lname",
    "password" : "password"
}
```

#### Status Codes
- 409 : Conflict someone has the same email
- 500 : Something went wrong most likley a database error or a bcrypt hashing error
- 201 : Everything was succesful and the user was created

