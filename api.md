# Api

## Authentication
### Register
```http
POST /auth/register
Content-Type : application/json
```

```http
   ### Expect { message: 'Wellcome to TestsAPI'}
   GET http://localhost:5001/

   ### Expect code 201 and info about the new user
   POST http://localhost:5001/user
   Content-Type: application/json

   {
     "name": "Pitossomo",
     "email": "pitossomos@hmail.com" 
   }
   
   ### Expect code 400
   POST http://localhost:5001/user
   Content-Type: application/json

   { 
     "name": "",
     "email": "" 
   }
 ```

