FROM keymetrics/pm2:12-alpine
WORKDIR /usr/src/cloudbanking-app
COPY package*.json package-lock.json  ./


ENV NPM_CONFIG_LOGLEVEL warn
RUN npm install

COPY . .

EXPOSE 3034

CMD [ "pm2-runtime", "start", "pm2.json" ]