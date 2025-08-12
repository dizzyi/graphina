/*!
# Custom Error Types

This module defines the custom error types for Graphina. These errors are used
throughout the library to show various failure conditions and provide specific
error information. Each exception implements the standard `std::error::Error`
and `std::fmt::Display` traits.

## Usage

Create an exception using the `Graphina::other` method and inspect it via its display implementation:

```rust
use graphina::core::exceptions::GraphinaError;
let err = GraphinaError::other("A generic error occurred.");
println!("{}", err); // GraphinaError { kind: Other, message: "a generic error" }
```
*/

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GraphinaError {
    pub kind: GraphinaErrorKind,
    pub message: String,
}

impl std::fmt::Display for GraphinaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg_struct = f.debug_struct("GraphinaError");
        dbg_struct.field("kind", &self.kind);
        dbg_struct.field("message", &self.message);
        dbg_struct.finish()
    }
}

impl Error for GraphinaError {}

impl GraphinaError {
    pub fn pointless(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::PointlessConcept,
            message: message.into(),
        }
    }
    pub fn algorithm_error(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::AlgorithmError,
            message: message.into(),
        }
    }
    pub fn unfeasible(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::Unfeasible,
            message: message.into(),
        }
    }
    pub fn no_path(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::NoPath,
            message: message.into(),
        }
    }
    pub fn no_cycle(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::NoCycle,
            message: message.into(),
        }
    }
    pub fn node_not_found(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::NodeNotFound,
            message: message.into(),
        }
    }
    pub fn edge_not_found(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::EdgeNotFound,
            message: message.into(),
        }
    }
    pub fn has_a_cycle(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::HasACycle,
            message: message.into(),
        }
    }
    pub fn unbounded(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::Unbounded,
            message: message.into(),
        }
    }
    pub fn ambiguous_solution(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::AmbiguousSolution,
            message: message.into(),
        }
    }
    pub fn exceeded_max_iteration(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::ExceededMaxIterations,
            message: message.into(),
        }
    }
    pub fn power_iteration_failed_converge(
        num_iterations: usize,
        message: impl Into<String>,
    ) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::PowerIterationFailedConvergence { num_iterations },
            message: message.into(),
        }
    }
    pub fn empty_graph(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::EmptyGraph,
            message: message.into(),
        }
    }
    pub fn parse_error(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::ParseError,
            message: message.into(),
        }
    }
    pub fn other(message: impl Into<String>) -> GraphinaError {
        GraphinaError {
            kind: GraphinaErrorKind::Other,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GraphinaErrorKind {
    /// Exception raised when a graph is provided to an algorithm that cannot use it.
    ///
    /// This error indicates that an algorithm received an invalid graph input like a null/empty graph.
    PointlessConcept,
    /// Exception for unexpected termination of algorithms.
    ///
    /// This error is used when an algorithm terminates unexpectedly.
    AlgorithmError,
    /// Exception raised when no feasible solution exists.
    ///
    /// This error indicates that an algorithm failed to find a viable solution (e.g., optimization).
    Unfeasible,
    /// Exception raised when no path exists between nodes.
    ///
    /// This error is returned when an algorithm determines that no valid path can be found.
    NoPath,
    /// Exception raised when no cycle exists in a graph.
    ///
    /// This error is used when an algorithm expects a cycle but none is found in the graph.
    NoCycle,
    /// Exception raised if a requested node is not found.
    ///
    /// This error is typically returned when an operation attempts to reference a non-existent node.
    NodeNotFound,
    /// Exception raised if a requested edge is not found.
    ///
    /// This error is typically returned when an operation attempts to reference a non-existent edge.
    EdgeNotFound,
    /// Exception raised if a graph has a cycle when an acyclic structure is expected.
    ///
    /// This error indicates that a cycle was found in a graph where it should not exist.
    HasACycle,
    /// Exception raised when an optimization problem is unbounded.
    ///
    /// This error is used when an algorithm detects that the solution is unbounded (e.g., linear programming).
    Unbounded,
    /// Exception raised for unimplemented algorithms for a given graph type.
    ///
    /// This error indicates that a requested algorithm or feature is not yet available.
    NotImplemented,
    /// Raised when more than one valid solution exists for an intermediary step.
    ///
    /// This error is used when an algorithm encounters ambiguity during a computational step (e.g., optimization).
    AmbiguousSolution,
    /// Raised if a loop iterates too many times without convergence.
    ///
    /// This error signals that an iterative algorithm has exceeded the allowed iteration limit.
    ExceededMaxIterations,
    /// Raised when the power iteration method fails to converge within the iteration limit (e.g., PageRank).
    ///
    /// This error includes the number of iterations attempted before failure.
    PowerIterationFailedConvergence { num_iterations: usize },
    /// Raised if tried to run algorithm that assume non-empty graph on empty graph  
    ///
    /// This error signals that the target graph is empty when it is assumed not to be.
    EmptyGraph,
    /// Raised if encounter error during parsing file format
    ///
    /// This error indicate that the target file might not be in the correct format
    ParseError,
    /// Raised if encounter unexpected
    ///
    /// This error return when the particlar error doesn't fall under any other error kind.
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphina_error() {
        let pair = [
            (
                GraphinaError::algorithm_error("algorithm error"),
                r#"GraphinaError { kind: AlgorithmError, message: "algorithm error" }"#,
            ),
            (
                GraphinaError::no_path("not path found"),
                r#"GraphinaError { kind: NoPath, message: "not path found" }"#,
            ),
            (
                GraphinaError::no_cycle("not cycle found"),
                r#"GraphinaError { kind: NoCycle, message: "not cycle found" }"#,
            ),
            (
                GraphinaError::node_not_found("cannot found node"),
                r#"GraphinaError { kind: NodeNotFound, message: "cannot found node" }"#,
            ),
            (
                GraphinaError::edge_not_found("cannot found edge"),
                r#"GraphinaError { kind: EdgeNotFound, message: "cannot found edge" }"#,
            ),
            (
                GraphinaError::empty_graph("graph is empty"),
                r#"GraphinaError { kind: EmptyGraph, message: "graph is empty" }"#,
            ),
            (
                GraphinaError::other("a generic error"),
                r#"GraphinaError { kind: Other, message: "a generic error" }"#,
            ),
        ];

        for (e, s) in pair {
            assert_eq!(format!("{}", e), s)
        }
    }
}
