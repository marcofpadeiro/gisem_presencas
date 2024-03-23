# Gisem Presencas
To start the project, you need to have installed either geckodriver or chromedriver and run it under port 4444.
Put your credentials in a credentials.txt file in the root of the project.

The file should have the following format:
```
username:password
```

## To deploy the project, you can use docker and run the following commands:
```
docker build -t gisem_presencas .
docker run -d gisem_presencas --name gisem_presencas
```
