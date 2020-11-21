(function() {var implementors = {};
implementors["buf_redux"] = [{"text":"impl&lt;R:&nbsp;Read, P:&nbsp;ReaderPolicy&gt; BufRead for BufReader&lt;R, P&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;Buf + Sized&gt; BufRead for Reader&lt;B&gt;","synthetic":false,"types":[]}];
implementors["multipart"] = [{"text":"impl&lt;M:&nbsp;ReadEntry&gt; BufRead for MultipartData&lt;M&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BufRead for DataReader&lt;'a&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()