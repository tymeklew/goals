# Authentication
## Register
Create a new account

### Request 
> **Content-Type : application/json** 
```http
POST /auth/register
```

### Expected form

```json
{
    "email" : "testing@gmail.com",
    "first_name" : "fname",
    "last_name" : "lname",
    "password" : "password"
}
```

### Status Codes
| Status Code | Description |
|-------------|-------------|
| 201         | Everything was succesful and the user has been added |
| 400         | Malformed parameters or other bad request |
| 409         | Conflict somebody else has the same email |
| 500         | Something went wrong on the server most likley a db or a bcrypt error|
## Login
### Request
Login to a previously created account
> **Content-Type : application/json** 
```http
POST /auth/login
```

### Expected Form
```json
{
    "email" : "testing@gmail.com",
    "password" : "password",
}
```

### Status Codes
| Status Code | Description| 
|-------------|------------|
| 200         | Everything was succesful and a session_id cookie will be sent back |
| 400         | Malformed request body |  
| 401         | Password was incorrect or user was not found | 
| 500         | Server error something went wrong |

# Goals 
Endpoints to create goals and others
## Create
### Request
> **Content-Type : application/json** 
```http
POST /goals/create
```
> **There must be a valid session_id cookie in the request otherwise will be sent back a 401 (Unathorized)** 

### Expected Form
```json
{
   "title" : "Workout",
   "description" : "Work out at least once a week"
}
```
### Status Codes
| Status Code | Description| 
|-------------|------------|
| 201         | Everything was succesful and a session_id cookie will be sent back |
| 400         | Malformed request body |  
| 401         | Password was incorrect or user was not found | 
| 500         | Server error something went wrong |
