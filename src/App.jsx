import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [path, setPath] = useState("");
  const [report, setReport] = useState(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function runAnalysis(e) {
    e.preventDefault();
    if (!path.trim()) return;

    setError("");
    setLoading(true);
    setReport(null);

    try {
      const result = await invoke("analyze_notes", { path });
      setReport(result);
    } catch (err) {
      const msg =
        typeof err === "string"
          ? err
          : "Không đọc được file. Kiểm tra lại đường dẫn.";
      setError(msg);
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="app">
      <header className="app-header">
        <h1>📊 Progress Analyzer</h1>
        <p className="subtitle">Theo dõi tiến độ NeetCode 150</p>
      </header>

      <form className="path-form" onSubmit={runAnalysis}>
        <input
          className="path-input"
          placeholder="Đường dẫn tới README.md..."
          value={path}
          onChange={(e) => setPath(e.target.value)}
          disabled={loading}
        />
        <button className="btn-primary" type="submit" disabled={loading}>
          {loading ? "Đang phân tích..." : "Phân tích"}
        </button>
      </form>

      {error && <p className="error-box">{error}</p>}

      {report && (
        <div className="report">
          <div className="summary-cards">
            <div className="card">
              <span className="card-value">{report.total}/150</span>
              <span className="card-label">Đã giải</span>
            </div>
            <div className="card">
              <span className="card-value">
                {report.progress_percent.toFixed(1)}%
              </span>
              <span className="card-label">Tiến độ</span>
            </div>
            <div className="card">
              <span className="card-value">
                {report.average_gap.toFixed(1)}d
              </span>
              <span className="card-label">Gap trung bình</span>
            </div>
          </div>

          <div className="table-grid">
            {/* By Category */}
            <section className="table-section">
              <h2>📚 By Category</h2>
              <table>
                <thead>
                  <tr>
                    <th>Category</th>
                    <th>Count</th>
                  </tr>
                </thead>
                <tbody>
                  {report.by_category.map(([cat, count]) => (
                    <tr key={cat}>
                      <td>{cat}</td>
                      <td>{count}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>

            {/* By Difficulty */}
            <section className="table-section">
              <h2>🎯 By Difficulty</h2>
              <table>
                <thead>
                  <tr>
                    <th>Difficulty</th>
                    <th>Count</th>
                  </tr>
                </thead>
                <tbody>
                  {report.by_difficulty.map(([diff, count]) => (
                    <tr key={diff} className={`diff-${diff.toLowerCase()}`}>
                      <td>{diff}</td>
                      <td>{count}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>

            {/* Top Approaches */}
            <section className="table-section">
              <h2>🛠️ Top Approaches</h2>
              <table>
                <thead>
                  <tr>
                    <th>Approach</th>
                    <th>Used</th>
                  </tr>
                </thead>
                <tbody>
                  {report.top_approaches.map(([app, count]) => (
                    <tr key={app}>
                      <td>{app}</td>
                      <td>{count}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </section>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;