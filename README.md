# Lamport ID

Lamport ID is a system designed for managing distributed identity using Lamport timestamps. 

## Prerequisites

Ensure that your system meets the following prerequisites:

- **Ubuntu-based system** (or compatible Linux distribution)
- **Sudo privileges** for system-wide installations

## Installation

### 1. Install System Dependencies

First, you'll need to install essential development tools and libraries:

```bash
sudo apt update
sudo apt install build-essential libssl-dev pkg-config
```

### 2. Install Redis

Redis is used as an in-memory data store. To install Redis on your system, follow these steps:

1. Add the Redis package repository:

   ```bash
   sudo apt-get install lsb-release curl gpg
   curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg
   sudo chmod 644 /usr/share/keyrings/redis-archive-keyring.gpg
   echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list
   sudo apt-get update
   sudo apt-get install redis
   ```

2. Start the Redis service:

   ```bash
   sudo systemctl enable redis-server
   sudo systemctl start redis-server
   ```

3. Test Redis to ensure it is working:

   ```bash
   redis-cli
   > ping
   ```

   If Redis is running correctly, it will respond with `PONG`.

### 3. Install PostgreSQL

PostgreSQL is used as the relational database for the system. To install PostgreSQL:

```bash
sudo apt install postgresql
```

#### Setup PostgreSQL Database

1. Switch to the PostgreSQL user:

   ```bash
   sudo su - postgres
   ```

2. Start the PostgreSQL shell:

   ```bash
   psql
   ```

3. Create the database user, database, and grant privileges:

   ```sql
   CREATE USER lamportid WITH PASSWORD '123456';
   ALTER USER lamportid WITH CREATEDB;
   CREATE DATABASE lamportid;
   GRANT ALL PRIVILEGES ON DATABASE lamportid TO lamportid;
   ```

4. Modify `pg_hba.conf` to allow password authentication:

   ```bash
   sudo nano /etc/postgresql/{version}/main/pg_hba.conf
   ```

   Change the following lines:

   ```conf
   local   all             all                                     md5
   host    all             all             127.0.0.1/32            md5
   ```

5. Restart PostgreSQL to apply the changes:

   ```bash
   sudo systemctl restart postgresql
   ```

### 4. Install Rust and Cargo

Rust is used for building the backend application. To install Rust and its package manager `cargo`, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, you may need to restart your terminal or reload the shell:

```bash
source $HOME/.cargo/env
```

### 5. Build the Application

Navigate to the root of the project and build the application:

```bash
cargo build --release
```

### 6. Install OpenResty (Nginx-based Web Server)

OpenResty is required for serving the application. To install OpenResty:

1. Add the OpenResty repository and install the package:

   ```bash
   wget -O - https://openresty.org/package/pubkey.gpg | sudo gpg --dearmor -o /usr/share/keyrings/openresty.gpg
   echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/openresty.gpg] http://openresty.org/package/ubuntu $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/openresty.list > /dev/null
   sudo apt update
   sudo apt install openresty
   ```

### 7. Configure Firewall

Allow incoming traffic on port 80 (HTTP):

```bash
sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT
```

### 8. Update `config.yaml`

Edit the `config.yaml` file to adjust settings such as database connection strings, Redis configurations, etc. Make sure it is configured according to your system's environment.

### 9. Migrate Database

Run the migration command to set up the database schema:

```bash
./target/release/acl-lamport-id migrate -c ./config.yaml
```

### 10. Run the Application with PM2

To run the application and manage it as a background process, we will use **PM2**.

1. Start the application using PM2:

   ```bash
   pm2 start ./target/release/acl-lamport-id --name "acl-lamport-id" -- run -c ./template/config_rel10.yaml
   ```

2. Set PM2 to start on boot:

   ```bash
   pm2 startup
   pm2 save
   ```

3. Restart the application (if needed):

   ```bash
   pm2 restart 0
   ```

### 11. Additional Setup

Make sure to review the application logs to verify that everything is working correctly:

```bash
pm2 logs acl-lamport-id
```

## Troubleshooting

- **Redis Not Starting**: Ensure that no other service is occupying the default Redis port (6379). You can modify the Redis configuration if needed.
- **PostgreSQL Errors**: If the database fails to start, check for permission issues or configuration errors in `pg_hba.conf`.

## License

Lamport ID is open-source software released under the [MIT License](LICENSE).
