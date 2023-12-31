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
### Return
If succesfull then will return a string with the id of the goal 

### Status Codes
| Status Code | Description| 
|-------------|------------|
| 201         | Everything was succesful and a session_id cookie will be sent back |
| 400         | Malformed request body |  
| 401         | Password was incorrect or user was not found | 
| 500         | Server error something went wrong |

## View
### Request
```http
GET /goals/view
```
```http
Get /goals/view/<id>
```
First request is for viewing all of the goals wherease the second one is for viewing a specific one with that id

### Return values 
```
#### Single
```json
{
    "id" : "id of goal",
    "user_id" : "id of user",
    "title: " title",
    "description" : "description"
}
```
#### Multiple
```json
[
{
    "id" : "id of goal",
    "user_id" : "id of user",
    "title": " title",
    "description" : "description"
},
]
```
### Status Codes
| Status Code | Description| 
|-------------|------------|
| 200         | Everything was succesful and a session_id cookie will be sent back |
| 401         | Unauthorized |
| 404         | Could not find the goal |
| 500         | Server error something went wrong |
