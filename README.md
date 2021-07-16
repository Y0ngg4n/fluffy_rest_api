
# Fluffy REST API

This is the REST API for the [Fluffyboard](https://github.com/Y0ngg4n/fluffy_board.git) Project.




![Logo](https://cdn.oblivioncoding.pro/fluffy_board/AppLogo.png)

    
## Badges
![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/yonggan/fluffy_rest_api?style=for-the-badge)

![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/yonggan/fluffy_rest_api?style=for-the-badge)

![Docker Pulls](https://img.shields.io/docker/pulls/yonggan/fluffy_rest_api?style=for-the-badge)

![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/yonggan/fluffy_rest_api?style=for-the-badge)
## Authors

- [@Y0ngg4n](https://github.com/Y0ngg4n)

  
## Demo

You can test out a demo of the REST API here:

[fluffy-board-rest-api.oblivioncoding.pro](fluffy-board-rest-api.oblivioncoding.pro)

⚠️ Warning this is the development REST API of [fluffyboard.obco.pro](fluffyboard.obco.pro) ⚠️
## Contributing

Contributions are always welcome!
## Deployment

To deploy this project you can use the docker container on dockerhub:

[yonggan/fluffy_rest_api](https://hub.docker.com/repository/docker/yonggan/fluffy_rest_api)

Checkout [docker-compose.yml](docker-compose.yml) to spin up a instance with dockerized scylla-db.

Currently scylla-db is used without authentication and sharding.
## Environment Variables

To run this project, you will need to add the following environment variables to your .env file or the [docker-compose.yml](docker-compose.yml)

`JWT_AUTH_SECRET`: Set this secret for Json Web Tokens. It is recommended to change this.

`SCYLLA_URI`: Your database connection URI. When used with docker-compose you can change to `scylla:9042`

  
## Roadmap

Checkout the [Github Project](https://github.com/Y0ngg4n/fluffy_rest_api/projects/2)

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)