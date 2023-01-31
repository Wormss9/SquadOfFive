FROM node:alpine as build

WORKDIR /app

COPY package*.json ./

RUN npm install

# copy project files and folders to the current working directory (i.e. 'app' folder)
COPY . .

# build app for production with minification
RUN npm run build

FROM alpine:latest
RUN apk update && apk upgrade
RUN apk add lighttpd
WORKDIR /app
COPY lighttpd.conf /etc/lighttpd/
COPY --from=build /app/dist/ /var/www/html/

EXPOSE 8080
CMD [ "lighttpd", "-D","-f","/etc/lighttpd/lighttpd.conf"]