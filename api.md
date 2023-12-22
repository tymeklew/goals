# Api

## Authentication
### Register

#### Request 
```http
POST /auth/register
```
> **Content-Type : application/json** \

#### Expected form

```json
{
    "email" : "testing@gmail.com",
    "first_name" : "fname",
    "last_name" : "lname",
    "password" : "password"
}
```

