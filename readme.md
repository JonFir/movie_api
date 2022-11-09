Simple movie api for the URLSession lesson

curl --location --request POST 'http://127.0.0.1:8080/auth/register' \
--header 'Content-Type: application/json' \
--data-raw '{
    "login": "admin",
    "password": "123456",
    "email": "admin@example.com"
}'

curl --location --request POST 'http://127.0.0.1:8080/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "login": "admin",
    "password": "123456",
    "email": "admin@example.com"
}'

curl --location --request POST 'http://127.0.0.1:8080/image/upload' \
--header 'Authorization: Bearer i4yO0lCy94mDpxGU8c4vecOUIOlvI96Wex6E1YtQpUkVlNBxXuoIf7XNf7vB96uuDC4dwMdJWVKwDUoy6fYuRMhCrQEY2GpIwIeqeCNw5DerepaJVH2IuiKoRZOIhg0ZPLj43X6bWdwQvXdNxTfXaMpNj0iOy42y7GuYAAj23SqmMx1FBIayiGKjKnv8AXJ77rSMvutyX7HBGhaNWnJA8P3ZQe4ssGo5JbITCWznqJ8HF18GCd0wi12wU8UStDN' \
--header 'Content-Type: image/jpeg' \
--data-binary '@/Users/e.elchev/Downloads/Cat03.jpeg'

curl --location --request GET 'http://127.0.0.1:8080/image/f2b481ca-0874-4109-96ae-e1c77f1ba324' \
--header 'Authorization: Bearer i4yO0lCy94mDpxGU8c4vecOUIOlvI96Wex6E1YtQpUkVlNBxXuoIf7XNf7vB96uuDC4dwMdJWVKwDUoy6fYuRMhCrQEY2GpIwIeqeCNw5DerepaJVH2IuiKoRZOIhg0ZPLj43X6bWdwQvXdNxTfXaMpNj0iOy42y7GuYAAj23SqmMx1FBIayiGKjKnv8AXJ77rSMvutyX7HBGhaNWnJA8P3ZQe4ssGo5JbITCWznqJ8HF18GCd0wi12wU8UStDN'

curl --location --request POST 'http://127.0.0.1:8080/movies/' \
--header 'Authorization: Bearer i4yO0lCy94mDpxGU8c4vecOUIOlvI96Wex6E1YtQpUkVlNBxXuoIf7XNf7vB96uuDC4dwMdJWVKwDUoy6fYuRMhCrQEY2GpIwIeqeCNw5DerepaJVH2IuiKoRZOIhg0ZPLj43X6bWdwQvXdNxTfXaMpNj0iOy42y7GuYAAj23SqmMx1FBIayiGKjKnv8AXJ77rSMvutyX7HBGhaNWnJA8P3ZQe4ssGo5JbITCWznqJ8HF18GCd0wi12wU8UStDN' \
--header 'Content-Type: application/json' \
--data-raw '{
    "movie": {
        
        "title": "test1",
        "director": "Ivan",
        "relise_date": 2017,
        "rating": 5,
        "poster_id": "55ba22ee-0855-41bc-83ff-a2e76a728081",
        "created_at": null
    }
}'