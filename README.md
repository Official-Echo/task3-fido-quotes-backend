# Завдання 3: бекенд для вебзастосунку, який дозволить користувачам ділитися цитатами

## Запуск

### База даних для способів 1 і 2

```bash
docker run --name quotes-backend-db \
  -e POSTGRES_USER=loco \
  -e POSTGRES_PASSWORD=loco \
  -e POSTGRES_DB=quotes_backend_development \
  -p 5432:5432 \
  -v quotes-backend-pgdata:/var/lib/postgresql \
  -d postgres:18-alpine
```

Перевірка на готовність:

```bash
docker exec -it quotes-backend-db pg_isready -U loco
```

Якщо є клонований репозиторій, можна ввести `docker compose up -d db` замість команди через `docker run`.

Щоб використати власний сервер PostgreSQL, задайте змінну середовища `DATABASE_URL` перед запуском застосунку.

### Спосіб 1: через Releases

1. Завантажте архів з релізів для своєї системи, розпакуйте `config/` та виконуваний файл у одну папку.

2. Запустіть базу даних (див. вище).

3. Запустіть застосунок.

    Linux:

    ```bash
    chmod +x quotes_backend-cli
    ./quotes_backend-cli start
    ```

    Windows:

    ```powershell
    .\quotes_backend-cli.exe start
    ```

4. Міграції буде виконано автоматично під час старту. Сервер доступний за адресою `http://localhost:5150`.

### Спосіб 2: через cargo

Потрібні git, Rust і Docker.

1. Завантажте репозиторій:

    ```bash
    git clone <адреса репозиторію>
    cd <назва репозиторію>
    ```

2. Запустіть базу даних:

    ```bash
    docker compose up -d db
    ```

3. Зберіть та запустіть застосунок:

    ```bash
    cargo run -- start
    ```

### Спосіб 3: через Docker Compose у середовищі development

У цьому способі нічого, крім Docker, встановлювати не треба.

```bash
docker compose up --build
```

Застосунок стартує після того, як база стане готовою і слухає порт `5150`. Дані лежать у томі `quotes-backend-pgdata-dev`. Для зупинки контейнера без видалення даних виконайте `docker compose down`, для видалення додайте прапорець `-v` в кінці.

### Спосіб 4: через Docker Compose у середовищі production

1. Створіть файл `.env` із шаблону та заповніть в ньому `POSTGRES_USER` й `POSTGRES_PASSWORD`. Без пароля база не ініціалізується. Якщо у вас нема власної адреси серверу, можете залишити дефолтний `localhost:5150`.

    ```bash
    cp .env.example .env
    ```

2. Опісля зберіть та запустіть:

    ```bash
    docker compose -f docker-compose.production.yml up --build -d
    ```

3. Для зупинки виконайте `docker compose -f docker-compose.production.yml down`.

### Перевірка запуску

```bash
curl http://localhost:5150/_ping
```

Успішна відповідь означає, що сервер працює.

## Використання API

Створення:

```bash
curl -i -X POST http://localhost:5150/quotes \
-H "Content-Type: application/json" \
-d '{"author": "Макарець Андрій", "text": "Чому мене ніхто не цитує в цитатах ФІ?"}'
```

Перегляд, оновлення та видалення:

```bash
curl -i http://localhost:5150/quotes/<id>

curl -i -X PUT http://localhost:5150/quotes/<id> \
-H "Content-Type: application/json" \
-d '{"author": "Медвідь Сергій", "text": "Аншланг"}'

curl -i -X DELETE http://localhost:5150/quotes/<id>
```

Список із фільтром за автором і пагінацією:

```bash
curl -i "http://localhost:5150/quotes?author=Медвідь%20Сергій&page=1&page_size=10"
```

```json
{
    "items": [{ "id": "…", "author": "Медвідь Сергій", "text": "…" }],
    "page": 1,
    "page_size": 10,
    "total_pages": 1,
    "total_items": 1
}
```

## Тести

Тести працюють із тестовою базою `quotes_backend_test`, яку створює `docker compose up -d db`.

```bash
docker compose up -d db
cargo test
```

Скрипт створення тестової бази виконується лише при першому створенні тому. Якщо том уже існував раніше, створіть базу вручну:

```bash
docker compose exec db psql -U loco -c "CREATE DATABASE quotes_backend_test;"
```

Навантажувальний тест, який масово створює, змінює та видаляє цитати, можна запустити через:

```bash
cargo test -- --ignored
```
