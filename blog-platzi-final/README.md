docker run --name postgres -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=here_my_secure_password -e POSTGRES_DB=lucira -v postgres_data:/var/lib/postgresql/data -d postgres:14

docker exec -it postgres psql -U postgres -d lucira
