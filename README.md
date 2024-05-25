# Rhubarb

Rhubarb is a lightweight query language designed to represent sets of asset permissions. It achieves this by creating an in-memory data structure that can be queried using simple, declarative statements. These queries generate granular permissions, encapsulating the essence of the intended access control.

Inspired by the paper: [Role-Based Access Controls (Ferraiolo and Kuhn, 1992)](https://arxiv.org/ftp/arxiv/papers/0903/0903.2171.pdf).

### Rhubarb is for teams that have the following considerations:

Complexity and Scale: If the organization has a large number of users and assets, leading to a high cardinality of permissions, Rhubarb’s ability to simplify and manage these permissions through a query language can be very beneficial.

Data Governance: For organizations with strict data governance policies and the need for clear, auditable access control, Rhubarb’s approach to permissions through declarative statements can enhance transparency and compliance.

Collaboration: Rhubarb aims to bridge the gap between the data team and the business side, making it easier for both to communicate and agree on access permissions. If such a gap is a significant issue, Rhubarb could improve collaboration and efficiency.

No Lock-In or Specific Infrastructure Requirements: Rhubarb is designed to be lightweight and infrastructure-agnostic, meaning it doesn't tie you to any specific platform or ecosystem. This flexibility can be advantageous for organizations that use a variety of tools and platforms, avoiding vendor lock-in and allowing for easier integration with existing systems.

# Problem Space

Rhubarb addresses the complexities of asset access control in large organizations, characterized by:

- Diverse User Base: Large organizations often have numerous users with varying levels of access across many assets, creating a challenging landscape for data access control systems.

- Data Governance: Organizations with a comprehensive data strategy, typically overseen by a Chief Data Officer (CDO) or Chief Technology Officer (CTO), are responsible for data governance policies. These policies are crucial, as the data team usually holds liability, while access requests originate from the business side.

- Scale of Permissions: In environments with over 1,000 users and assets, the potential number of atomic-level permissions can reach into the millions (m x n). Managing and auditing these permissions can overwhelm the data team, detracting from other productive tasks.

- Organizational Divide: A gap often exists between the traditional data team (focused on data operations and governance) and the business side (employees and line managers). Asset access requests typically come from the business, cross this divide, and are executed by the data team if they align with data policies.

- Visibility Challenges: Requestors on the business side lack visibility into the high cardinality of assets, which is known only to the data team. Conversely, the data team lacks visibility into the high cardinality of personnel, known only to management.

- Complexity Management: The intersection of these high cardinalities results in extreme and unmanageable complexity. A mediating solution is needed that both sides can agree upon, abstracting away the cardinality issues on either side.

# Rhubarb's solution

Rhubarb provides a balanced solution, enhancing readability and expressiveness, allowing both the data team and business side to manage and communicate access permissions effectively. The aim is for the two teams to agree on a manageable number of statements (5-50), which can be easily reviewed and agreed upon.

```
GRANT READ ON (schema:tax EXCEPT table:sensitive_audit) TO (department:tax AND (designation:partner OR designation:senior))
```

Rhubarb uses an SQL-like query language, which includes:

- `GRANT READ ON`: Represents an access operation.
- Set notation collections before and after TO:
    - `(schema:tax EXCEPT table:sensitive_audit)`: Abstracts cardinality on the asset side.
    - `(department:tax AND (designation:partner OR designation:senior))`: Abstracts cardinality on the user side.


Using set language allows for greater expressiveness than hierarchical classification (e.g., assigning permissions to an entire division or department). By leveraging RBAC principles and operations like union, intersection, and complement, complex selections can be expressed in a human-readable format.

Readability is crucial, as Rhubarb operates at the intersection of the data and business teams. Expressiveness is equally important, enabling the creation of thousands of granular permissions from a single statement.

# example queries
```
(department:tax AND designation:partner)

((division:product OR division:finance) AND designation:intern)

(division:product AND division:finance) => empty set

((department:strategy AND security_clearance:true) AND designation:senior)

// grant to personnel from these teams but only partners or seniors or junior staff with clearance
((department:r&d OR division:strategy) AND (security_clearance:true OR (designation:partner OR designation:senior)))
```
observe that intersections act mostly as filters - it's the unions which give some interesting capabilities
