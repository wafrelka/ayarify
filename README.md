# Ayarify

## Usage

```sh-session
$ cat testdata/sample.html
<!DOCTYPE html>
<html>
    <head></head>
    <body>
        <h2>header1</h2>
        <p>para1</p>
        <div data-ayarify="">
            <h2 class="cls">header2</h2>
            <p>para2</p>
            <h4>header3</h4>
            <p>para3</p>
            <h3>header4</h3>
            <p>para4</p>
            <h2>header5</h2>
            <p>para5</p>
        </div>
    </body>
</html>

$ cargo run 2> /dev/null < testdata/sample.html
<!DOCTYPE html><html><head></head>
    <body>
        <h2>header1</h2>
        <p>para1</p>
        <div data-ayarify="">
            <div class="cls"><h2>header2</h2>
            <p>para2</p>
            <div><h4>header3</h4>
            <p>para3</p>
            </div><div><h3>header4</h3>
            <p>para4</p>
            </div></div><div><h2>header5</h2>
            <p>para5</p>
        </div></div>


</body></html>
```
