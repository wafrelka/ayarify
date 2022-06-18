# Ayarify

## Usage

```sh-session
$ cat sample.html
<html>
<head></head>
<body>
<h2 class="cls">header1</h2><p>para1</p>
<h4>header3</h4><p>para3</p>
<h3>header4</h3><p>para4</p>
<h2>header5</h2><p>para5</p>
</body>
</html>

$ cat sample.html | cargo run --release 2>/dev/null
<html><head></head>
<body>
<div class="cls"><h2>header1</h2><p>para1</p>
<div><h4>header3</h4><p>para3</p>
</div><div><h3>header4</h3><p>para4</p>
</div></div><div><h2>header5</h2><p>para5</p>


</div></body></html>
```
