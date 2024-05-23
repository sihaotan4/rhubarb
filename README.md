# Rhubarb

Rhubarb is a lightweight Role-Based Access Control (RBAC) engine designed to create and manage granular permissions using a simple and intuitive language. It allows developers and administrators to define precise access controls through easy-to-write declarative statements, leveraging a domain-specific language that supports set operations to resolve complex groupings.

This project is inspired by the paper: [Role-Based Access Controls (Ferraiolo and Kuhn, 1992)](https://arxiv.org/ftp/arxiv/papers/0903/0903.2171.pdf).

## Key Features

1. Lightweight and Efficient

    - Minimal Overhead: Rhubarb is designed to be lightweight, ensuring it doesn't add significant overhead to your application.
    - High Performance: Optimized for quick resolution of granular permissions to maintain application performance.

2. Simple Declarative Language

    - Intuitive Syntax: Write simple, clear statements to define roles and permissions.
    - Human-Readable: The syntax is designed to be easily understandable, ensuring that permissions are reviewable by a non-technical audience.

3. Granular Permissions

    - Fine-Grained Control: Define specific permissions at a very detailed level to ensure precise access control. Rhubarb breaks down permissions to their simplest, most specific form. This ensures that each permission granted is as precise and granular as possible.
    - Customizable Roles: Create roles tailored to your application's unique requirements.

4. Domain-Specific Language (DSL)

    - Set Notation for Permissions: Use a DSL similar to set notation (set algebra) to resolve permissions and define complex groupings or sets of users.
    - Advanced Groupings: Define permissions based on intersections, unions, and differences of sets, allowing for sophisticated access control rules.

# inspiration

[Role-Based Access Controls (Ferraiolo and Kuhn, 1992)](https://arxiv.org/ftp/arxiv/papers/0903/0903.2171.pdf)


