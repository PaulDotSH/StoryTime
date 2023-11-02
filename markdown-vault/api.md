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
body: string
```
Example:
```
{
    "body": "Test story body!!!"
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
    "child_cannon_time": "DateTime",
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
    "child_cannon_time": "DateTime",
    "parent": "Optional<Uuid>",
    "child": "Optional<Uuid>",
    "score": "Int"
  },
  ...
]
```
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
