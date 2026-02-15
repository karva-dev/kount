# CLI Reference

## kount

Count lines in files and directories

<h3 class="cli-reference">Usage</h3>

```
kount [OPTIONS] [PATHS]...
```

<h3 class="cli-reference">Arguments</h3>

<dl class="cli-reference"><dt id="kount--paths"><a href="#kount--paths"><code>PATHS</code></a></dt><dd><p>Files or directories to count (default: current directory)</p>
</dd></dl>

<h3 class="cli-reference">Options</h3>

<dl class="cli-reference"><dt id="kount--ext"><a href="#kount--ext"><code>--ext</code></a>, <code>-e</code> <i>ext</i></dt><dd><p>Filter by extension (comma-separated: rs,py,js)</p>
</dd><dt id="kount--glob"><a href="#kount--glob"><code>--glob</code></a>, <code>-g</code> <i>glob</i></dt><dd><p>Filter by glob pattern (repeatable)</p>
</dd><dt id="kount--help"><a href="#kount--help"><code>--help</code></a>, <code>-h</code></dt><dd><p>Print help</p>
</dd><dt id="kount--json"><a href="#kount--json"><code>--json</code></a></dt><dd><p>Output as JSON</p>
</dd><dt id="kount--no-ignore"><a href="#kount--no-ignore"><code>--no-ignore</code></a></dt><dd><p>Include hidden files, ignore .gitignore</p>
</dd><dt id="kount--sort"><a href="#kount--sort"><code>--sort</code></a>, <code>-s</code> <i>sort</i></dt><dd><p>Sort order [default: lines]</p>
<p>[default: lines]</p><p>Possible values:</p>
<ul>
<li><code>lines</code></li>
<li><code>name</code></li>
<li><code>none</code></li>
</ul></dd><dt id="kount--summary"><a href="#kount--summary"><code>--summary</code></a></dt><dd><p>Show only totals and per-extension breakdown</p>
</dd><dt id="kount--version"><a href="#kount--version"><code>--version</code></a>, <code>-V</code></dt><dd><p>Print version</p>
</dd></dl>

