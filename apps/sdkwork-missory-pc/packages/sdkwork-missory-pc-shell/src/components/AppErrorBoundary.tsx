import { Component, type ReactNode } from "react";

interface ErrorBoundaryProps {
  children: ReactNode;
}

interface ErrorBoundaryState {
  error: Error | null;
}

/**
 * Root render-error boundary: a crashed route renders the error message with
 * a reload action instead of unmounting the whole console to a blank page.
 */
export class AppErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  state: ErrorBoundaryState = { error: null };

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { error };
  }

  componentDidCatch(error: Error): void {
    console.error("render error", error);
  }

  render(): ReactNode {
    const { error } = this.state;
    if (error) {
      return (
        <div style={{ maxWidth: 720, margin: "80px auto", padding: 24, fontFamily: "sans-serif" }}>
          <h2 style={{ margin: "0 0 12px" }}>页面出错了</h2>
          <pre
            style={{
              whiteSpace: "pre-wrap",
              fontSize: 13,
              background: "#f6f6f6",
              padding: 12,
              borderRadius: 8,
            }}
          >
            {error.message}
          </pre>
          <button
            type="button"
            className="sdk-button"
            onClick={() => this.setState({ error: null })}
          >
            重试
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}
