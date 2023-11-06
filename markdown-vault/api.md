## Auth
### Register
#### Endpoint:
`POST` `/register`
#### Input:
Type: `Json`
```
email: string,
username: string,
password: string
```
Example:
```
{
    "email": "test@storytime.com",
    "username": "test",
    "password": "asd123"
}
```
#### Output:
```
Status code 500
OR
SET AUTH COOKIE and Redirect to homepage
```

### Login
#### Endpoint:
`POST` `/login`
#### Input:
Type: `Json`
```
username: string,
password: string
```
Example:
```
{
    "username": "test",
    "password": "asd123"
}
```
#### Output:
```
Status code 500
OR
SET AUTH COOKIE and Redirect to homepage
```

## Story snippets
### Post new snippet
#### Endpoint:
`POST` `/snippets/new`
#### Input:
Type: `Json`
```
body: string,
is_final: bool
```
#### IMPORTANT!!
If the new index is smaller than MIN_INDEX the variable is_final is set automatically to false, also if the new index is equals to MAX_INDEX the is_final is set automatically too

Example:
```
{
    "body": "Test story body!!!",
    is_final: false
}
```
#### Output:
```
Status code 500 (lack of permission or db error)
OR
Status code 200 with the story id
```
### Edit snippet
#### Endpoint:
`PUT` `/snippets/:id`
#### Input:
Type: `Json`
```
body: string
```
Example:
```
{
    "body": "New story body!!!"
}
```
#### Output:
```
Status code 500 (lack of permission or db error)
OR
Status code 200
```
### Add snippet continuation
#### Endpoint:
`POST` `/snippets/:id/new`
#### Input:
Type: `Json`
```
body: string
```
Example:
```
{
    "body": "New story body!!!"
}
```
#### Output:
```
Status code 500 (lack of permission or db error)
OR
Status code 200 with the new story snippet id
```
### Get snippet info
#### Endpoint:
`GET` `/snippets/:id/`
#### Output:
```
Status code 500 (db error)
```
OR
```json
{
    "id": "Uuid",
    "writer": "String",
    "body": "String",
    "created": "DateTime",
    "modified": "Optional<DateTime>",
    "child_cannon_time": "Optional<DateTime>",
    "parent": "Optional<Uuid>",
    "child": "Optional<Uuid>",
    "score": "int"
}
```
### Get snippet children
#### Endpoint:
`GET` `/snippets/:id/children`
#### Input:
Type: `Query parameter`
```
last_score: Optional<Int>
```
#### Output:
```
Status code 500 (db error)
```
OR
```json
[
  {
    "id": "Uuid",
    "writer": "String",
    "body": "String",
    "created": "DateTime",
    "modified": "Optional<DateTime>",
    "child_cannon_time": "Optional<DateTime>",
    "parent": "Optional<Uuid>",
    "child": "Optional<Uuid>",
    "score": "Int"
  },
  ...
]
```
If child_cannon_time isn't null, it means it's canon.
## Comments
### Post new comment
#### Endpoint:
`POST` `/snippets/:id/comments/new`
#### Input:
Type: `Json`
```
body: string
```
Example:
```
{
    "body": "Test comment"
}
```
#### Output:
```
Status code 500 (lack of permission or db error)
OR
Status code 200 with the comment id
```
### Edit comment
#### Endpoint:
`PUT` `/comments/:id`
#### Input:
Type: `Json`
```
body: string
```
Example:
```
{
    "body": "Edited comment"
}
```
#### Output:
```
Status code 500 (lack of permission or db error)
OR
Status code 200
```
### View snippet comments
#### Endpoint:
`PUT` `/snippets/:id/comments`
#### Input:
Type: `Query parameter`
```
last_score: Optional<Int>
```
#### Output:
```
Status code 500 (lack of permission or db error)
```
OR
```json
[
    {
        "id": "Uuid",
        "writer": "String",
        "body": "String",
        "created": "DateTime",
        "modified": "Optional<DateTime>",
        "score": "Int"
    }
]
```
### Vote snippet
#### Endpoint:
`POST` `/snippets/:id/vote`
#### Input:
Type: `Json`
```
"Up"
```
or
```
"Down"
```
#### Output:
```
Status code 200
OR
Status code 500 (lack of permission or db error)
```
### Remove snippet vote
#### Endpoint:
`DELETE` `/snippets/:id/vote`
#### Output:
```
Status code 200
OR
Status code 500 (lack of permission, doesnt exist etc)
```
## Email Verification
### Send code verification
#### Endpoint:
`POST` `/resend`
#### Input:
Type: `Json`
```
email: string
```
Example:
```
{
    "email": "test.name@gmail.com"
}
```
#### Output:
```
Status code 500 (email doesnt exist or db error)
OR
Status code 200
```
### Verify account
#### Endpoint:
`POST` `/confirm/:code`
#### Output:
```
Status code 500 (code expired or db error)
OR
Status code 200
```