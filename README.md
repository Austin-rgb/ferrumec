## Ferrumec - a Rust Microservice Ecosystem

Composable microservices for building high-performance business and enterprise software.

This ecosystem provides a set of generic, decoupled microservices that can be composed to create fast, scalable, and maintainable applications—from SMB projects to enterprise-grade platforms.

---

### 🌐 Vision

Many platforms lock you into opinionated workflows or heavy frameworks.
Our ecosystem is designed to be:

- Generic: Services expose primitives, not business logic.

- Decoupled: Each service owns its domain, communicates via APIs or events.

- High-performance: Built in Rust with Actix-web for speed and low overhead.

- Composable: Mix, match, and extend services for any project.

- Enterprise-ready: Multi-tenancy, audit logging, and compliance hooks included.

---

### 🔹 Core Principles

1. One capability per service
   Each microservice does one thing well—no business-specific assumptions.

2. Event-driven and API-first
   Services emit events and provide clear APIs. Integration is seamless.

3. Schema-agnostic design
   Data structures are generic; workflows are defined externally.

4. Composable and reusable
   Deploy independently, integrate anywhere, and extend with plugins or custom logic.

5. Developer-first experience
   SDKs, reference apps, and clear OpenAPI contracts make adoption frictionless.

---

### 🚀 Core Microservices

Service Description

- [auth-service](github.com/Austin-rgb/auth_service) Identity, OAuth2/OIDC, JWT, RBAC/ABAC, multi-tenant
- [tenant-service](github.com/Austin-rgb/tenant) Tenant lifecycle management, plan metadata, feature flags
- [notification-service](github.com/Austin-rgb/messages) Email, SMS, webhooks, template-driven, provider-agnostic
- workflow-service State machine engine for async and human workflows
- audit-log-service Append-only logs, queryable by tenant/entity, compliance-ready
- file-service Object metadata storage with S3/GCS adapter support

> More services coming: analytics, reporting, policy engine, integration connectors.

---

### ⚙️ Architecture Highlights

Rust + Actix-web for ultra-low latency

Async-first, non-blocking I/O

Database per service, strict domain ownership

Event-driven communication for decoupled workflows

Multi-tenant from the start, enterprise compliance ready

---

💡 Getting Started

Prerequisites

Rust 1.80+

Docker & Docker Compose (for local development)

PostgreSQL / Redis / Kafka (depending on service)

Clone the ecosystem

git clone https://github.com/Austin-rgb/ferrumec.git
cd ferrumec

Run locally

docker-compose up
cargo run --bin auth-service
cargo run --bin tenant-service

# Add other services as needed

Explore APIs

Each service exposes an OpenAPI spec at /api-docs.
You can also generate client SDKs using cargo openapi or openapi-generator.

---

🌱 Why This Ecosystem?

Faster development: Don’t reinvent authentication, workflow engines, or notifications.

Flexible scaling: Move from SMB project to enterprise-grade architecture seamlessly.

High performance: Rust and Actix-web ensure low latency and minimal resource usage.

Future-proof: Modular design allows adding services without breaking existing apps.

---

📦 Contributing

We welcome contributions!

Add new services

Improve documentation

Build client SDKs for other languages

Provide templates for common workflows

Please read CONTRIBUTING.md for guidelines.

---

📜 License

MIT License © 2025 Your Organization

---

🔗 Links

Documentation

Roadmap

Changelog
