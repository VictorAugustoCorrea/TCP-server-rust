<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>TCP Server - Rust</title>
</head>
<body>

<h1>TCP Server - Rust</h1>

<p>
<img src="https://img.shields.io/badge/Rust-2021-orange.svg" alt="Rust Badge">
<img src="https://img.shields.io/badge/Networking-TCP-blue.svg" alt="TCP Badge">
<img src="https://img.shields.io/badge/Concurrency-Threads-purple.svg" alt="Concurrency Badge">
</p>

<p>
A <strong>simple TCP server</strong> developed in <strong>Rust</strong>.<br>
This project demonstrates how to create a TCP listener, accept incoming connections,
handle multiple clients concurrently using threads, and safely manage memory using Rust's
ownership model.
</p>

<p>
The main goal is to explore <strong>low-level networking fundamentals</strong>,
<strong>concurrency</strong>, and <strong>safe systems programming</strong> with Rust.
</p>

<hr>

<h2>📋 Features</h2>
<ul>
  <li><strong>TCP socket server</strong> using <code>TcpListener</code></li>
  <li><strong>Multiple concurrent clients</strong> (thread-per-connection model)</li>
  <li><strong>Safe ownership transfer</strong> across threads</li>
  <li><strong>Modular code organization</strong></li>
  <li><strong>Blocking I/O</strong> with explicit read/write handling</li>
  <li><strong>Clear separation of concerns</strong> between server and client handler</li>
</ul>

<hr>

<h2>🛠️ Technologies Used</h2>
<ul>
  <li><strong>Language:</strong> Rust (Edition 2021)</li>
  <li><strong>Standard Library:</strong>
    <ul>
      <li><code>std::net::TcpListener</code> — listening for TCP connections</li>
      <li><code>std::net::TcpStream</code> — client communication</li>
      <li><code>std::thread</code> — concurrency model</li>
      <li><code>std::io</code> — reading and writing streams</li>
    </ul>
  </li>
</ul>

<hr>

<h2>🚀 How to Compile and Run</h2>

<h3>Prerequisites</h3>
<ul>
  <li>Rust installed (via <code>rustup</code>)</li>
  <li>Cargo properly configured</li>
</ul>

<h3>Steps</h3>

<p><strong>1. Clone the repository:</strong></p>
<pre><code>git clone git@github.com:VictorAugustoCorrea/TCP-server-rust.git
cd TCP-server-rust
</code></pre>

<p><strong>2. Build the project:</strong></p>
<pre><code>cargo build
</code></pre>

<p><strong>3. Run the server:</strong></p>
<pre><code>cargo run
</code></pre>

<hr>

<h2>🎯 Example Usage</h2>

<p>After starting the server, connect using <code>telnet</code> or <code>netcat</code>:</p>

<pre><code>telnet 127.0.0.1 8080
</code></pre>

<p>Server output example:</p>

<pre><code>Server listening on 127.0.0.1:8080
Received request: Hello from client
</code></pre>

<p>Client receives:</p>

<pre><code>Hello, Client!
</code></pre>

<hr>

<h2>⚠️ Current Limitations</h2>
<ul>
  <li>Thread-per-connection model (does not scale to thousands of clients)</li>
  <li>Blocking I/O</li>
  <li>Single fixed response</li>
  <li>No protocol parsing (not HTTP)</li>
</ul>

<hr>

<h2>📈 Future Improvements</h2>
<ul>
  <li>Thread pool implementation</li>
  <li>Async version using Tokio</li>
  <li>HTTP protocol support</li>
  <li>Timeouts and graceful shutdown</li>
  <li>Metrics and structured logging</li>
</ul>

<hr>

<h2>📄 License</h2>
<p>This project is intended for educational and portfolio purposes.</p>

</body>
</html>
