# Graph Construction

The graph defines the structure of your workflow - how tasks connect and the paths execution can take.

## GraphBuilder

Use `GraphBuilder` to construct workflows:

```rust
use graph_flow::{GraphBuilder, Graph, Task};
use std::sync::Arc;

let graph = GraphBuilder::new("my_workflow")
    .add_task(task1)
    .add_task(task2)
    .add_edge(task1.id(), task2.id())
    .build();
```

## Adding Tasks

Tasks are the nodes in your workflow graph:

```rust
// Create task instances wrapped in Arc
let task1 = Arc::new(FirstTask);
let task2 = Arc::new(SecondTask);
let task3 = Arc::new(ThirdTask);

// Add to graph
let graph = GraphBuilder::new("workflow")
    .add_task(task1.clone())
    .add_task(task2.clone())
    .add_task(task3.clone())
    .build();
```

The first task added becomes the start task automatically.

## Connecting Tasks

### Linear Edges

Connect tasks in sequence:

```rust
let graph = GraphBuilder::new("linear_workflow")
    .add_task(task_a)
    .add_task(task_b)
    .add_task(task_c)
    .add_edge(task_a.id(), task_b.id())  // A → B
    .add_edge(task_b.id(), task_c.id())  // B → C
    .build();
```

### Setting Start Task

Explicitly set which task starts the workflow:

```rust
let graph = GraphBuilder::new("workflow")
    .add_task(start)
    .add_task(middle)
    .add_task(end)
    .set_start_task(start.id())  // Explicit start
    .add_edge(start.id(), middle.id())
    .add_edge(middle.id(), end.id())
    .build();
```

## Conditional Edges

Conditional edges allow dynamic routing based on runtime data:

```rust
use graph_flow::{GraphBuilder, Context};

let graph = GraphBuilder::new("conditional_workflow")
    .add_task(classifier_task)
    .add_task(positive_task)
    .add_task(negative_task)
    .add_conditional_edge(
        classifier_task.id(),                    // From task
        |ctx: &Context| {                        // Condition function
            ctx.get_sync::<String>("sentiment")
                .map(|s| s == "positive")
                .unwrap_or(false)
        },
        positive_task.id(),                       // Yes branch
        negative_task.id()                        // No branch
    )
    .build();
```

### How Conditional Edges Work

```
Task A
   │
   ├── condition returns true ──► Task B (yes branch)
   │
   └── condition returns false ──► Task C (no branch)
```

The condition function:
- Receives the Context
- Returns `true` or `false`
- Is evaluated at runtime

### Multiple Conditions

Chain conditions for complex routing:

```rust
GraphBuilder::new("multi_conditional")
    .add_task(classify_task)
    .add_task(car_task)
    .add_task(apartment_task)
    .add_task(other_task)
    .add_conditional_edge(
        classify_task.id(),
        |ctx| ctx.get_sync::<String>("type").unwrap_or_default() == "car",
        car_task.id(),
        apartment_task.id()  // This becomes the next conditional
    )
    // Then add another conditional from apartment_task to handle "other"
```

## Workflow Patterns

### Linear Workflow

Simple step-by-step execution:

```rust
// A → B → C
let graph = GraphBuilder::new("linear")
    .add_task(step_a)
    .add_task(step_b)
    .add_task(step_c)
    .add_edge(step_a.id(), step_b.id())
    .add_edge(step_b.id(), step_c.id())
    .build();
```

### Branching Workflow

Different paths based on conditions:

```rust
//        → Task B (if condition)
// Task A 
//        → Task C (if not)
let graph = GraphBuilder::new("branching")
    .add_task(decision)
    .add_task(task_b)
    .add_task(task_c)
    .add_conditional_edge(
        decision.id(),
        |ctx| ctx.get_sync::<bool>("condition").unwrap_or(false),
        task_b.id(),
        task_c.id()
    )
    .build();
```

### Merging Workflow

Multiple paths converge:

```rust
//    → Task B ─┐
// Task A       │
//    → Task C ─┴─► Task D
let graph = GraphBuilder::new("merging")
    .add_task(task_a)
    .add_task(task_b)
    .add_task(task_c)
    .add_task(task_d)
    .add_conditional_edge(task_a.id(), |_| true, task_b.id(), task_c.id())
    .add_edge(task_b.id(), task_d.id())
    .add_edge(task_c.id(), task_d.id())
    .build();
```

### Diamond Pattern

Classic diamond decision pattern:

```rust
//        → B →
//      ↗     ↖
//    A         D
//      ↘     ↗
//        → C →
let graph = GraphBuilder::new("diamond")
    .add_task(start)
    .add_task(branch_b)
    .add_task(branch_c)
    .add_task(end)
    .add_conditional_edge(
        start.id(),
        |ctx| ctx.get_sync::<bool>("choose_b").unwrap_or(true),
        branch_b.id(),
        branch_c.id()
    )
    .add_edge(branch_b.id(), end.id())
    .add_edge(branch_c.id(), end.id())
    .build();
```

### Loop Pattern

Create loops with GoTo:

```rust
// A → B → C (if bad) → back to B
//              → (if good) → D
let graph = GraphBuilder::new("loop")
    .add_task(input)
    .add_task(validate)
    .add_task(process)
    .add_task(done)
    .add_edge(input.id(), validate.id())
    .add_conditional_edge(
        validate.id(),
        |ctx| ctx.get_sync::<bool>("valid").unwrap_or(false),
        process.id(),
        input.id()  // Loop back
    )
    .add_edge(process.id(), done.id())
    .build();
```

## Graph Validation

The builder validates your graph:

```rust
let graph = GraphBuilder::new("workflow")
    .add_task(task1)
    .add_task(task2)
    .build();

// Warnings for orphaned tasks
// (tasks with no incoming or outgoing edges)
```

## Complete Example

Building the insurance claims workflow:

```rust
use graph_flow::{GraphBuilder, NextAction};

let initial_query = Arc::new(InitialClaimQueryTask);
let classifier = Arc::new(InsuranceTypeClassifierTask);
let car_details = Arc::new(CarInsuranceDetailsTask);
let apartment_details = Arc::new(ApartmentInsuranceDetailsTask);
let validator = Arc::new(SmartClaimValidatorTask);
let final_summary = Arc::new(FinalSummaryTask);

let graph = GraphBuilder::new("insurance_claims")
    .add_task(initial_query.clone())
    .add_task(classifier.clone())
    .add_task(car_details.clone())
    .add_task(apartment_details.clone())
    .add_task(validator.clone())
    .add_task(final_summary.clone())
    
    // Initial query → Classifier
    .add_edge(initial_query.id(), classifier.id())
    
    // Classifier → Car OR Apartment (conditional)
    .add_conditional_edge(
        classifier.id(),
        |ctx| ctx.get_sync::<String>("insurance_type")
            .map(|t| t == "car")
            .unwrap_or(false),
        car_details.id(),
        apartment_details.id()
    )
    
    // Both lead to validator
    .add_edge(car_details.id(), validator.id())
    .add_edge(apartment_details.id(), validator.id())
    
    // Validator → Final summary (always ends here)
    .add_edge(validator.id(), final_summary.id())
    
    .build();
```

This creates:

```
InitialClaimQuery → InsuranceTypeClassifier
                           │
              ┌────────────┴────────────┐
              │                         │
           (car)                    (apartment)
              │                         │
       CarInsuranceDetails    ApartmentInsuranceDetails
              └────────────┬────────────┘
                           │
                    SmartClaimValidator
                           │
                    FinalSummary
```

## Next Steps

- [Session Management](./session-management.md) - Managing workflow state
- [Execution Control](./execution-control.md) - Understanding NextAction
- [Examples](./examples.md) - Complete workflow examples
