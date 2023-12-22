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
| Status Code | Description |
|-------------|-------------|
| 201        | Everything was succesful and the user has been added |
| 400        | Malformed parameters or other bad request |
| 409        | Conflict somebody else has the same email |
| 500        | Something went wrong on the server most likley a db or a bcrypt error |

