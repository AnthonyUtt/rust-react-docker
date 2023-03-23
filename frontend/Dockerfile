FROM node:18.14.0

ARG PORT

WORKDIR /usr/src/app

COPY . .

RUN npm install -g pnpm

RUN pnpm install --frozen-lockfile

EXPOSE $PORT:$PORT

CMD [ "pnpm", "run", "develop" ]
