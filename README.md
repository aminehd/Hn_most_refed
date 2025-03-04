# HN Most Referenced

A Rust-based system to read from APIs, process data through Kafka, and store hourly aggregates in PostgreSQL.

## Architecture

- **Rust API Server**: Handles HTTP requests and communicates with Kafka
- **Kafka**: Message broker for reliable data processing
- **PostgreSQL**: Stores aggregated data
- **Portainer**: Container management UI

## Development Setup

### Prerequisites

- Docker and Docker Compose
- Rust (for local development)

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/hn_most_refed.git
   cd hn_most_refed
   ```

2. Start the services:
   ```bash
   docker-compose up -d
   ```

3. Access the services:
   - Rust API: http://localhost:8080
   - Portainer: http://localhost:9000
   - Kafka: kafka:9092 (internal) or localhost:29092 (external)
   - PostgreSQL: postgres:5432

### API Endpoints

- `GET /`: Hello world
- `GET /health`: Health check
- `POST /kafka`: Send a test message to Kafka

## Production Deployment (Digital Ocean)

1. Create a new droplet on Digital Ocean (recommended: Basic Droplet with 2GB RAM / 1 CPU)

2. SSH into your droplet:
   ```bash
   ssh root@your-droplet-ip
   ```

3. Install Docker and Docker Compose:
   ```bash
   curl -fsSL https://get.docker.com -o get-docker.sh
   sh get-docker.sh
   
   apt-get install -y docker-compose-plugin
   ```

4. Clone the repository:
   ```bash
   git clone https://github.com/your-username/hn_most_refed.git
   cd hn_most_refed
   ```

5. Start the services using the production configuration:
   ```bash
   docker compose -f docker-compose.prod.yml up -d
   ```

6. Set up firewall rules to allow traffic to ports 8080 (API) and 9000 (Portainer):
   ```bash
   ufw allow 22/tcp
   ufw allow 8080/tcp
   ufw allow 9000/tcp
   ufw enable
   ```

## Future Work

- Implement hourly aggregation logic
- Add authentication to API endpoints
- Set up data retention policies
- Implement monitoring and alerting
