FROM node
WORKDIR /usr/src/app
RUN npm install pg express ejs
COPY files/. /usr/src/app/
ENV PORT=8000 PG_HOST="localhost" PG_PORT="5432" PG_DB="testdb" PG_USER="postgres" PG_PASS="postgres"
EXPOSE $PORT 
ENTRYPOINT [ "node" , "server.js" ]
