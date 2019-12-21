function lessTheOppacity(ele) {
    var box = document.getElementById(ele);
    var x = 1;

    (function next(v) {
        v -= .05;
        box.style.opacity = v;
        if (++x < 20) {
            setTimeout(next, 200, v);
            if (x == 19) {
                box.style.display = "none";
            }
        }
    })(x);
}

window.addEventListener('load', function() {

    const sleep = (milliseconds) => {
        return new Promise(resolve => this.setTimeout(resolve, milliseconds))
    }

    absolute_div = "login-moodle-id";

    this.document.getElementById(absolute_div).style.pointerEvents = "none";

    sleep(2000).then(() => {
        lessTheOppacity(absolute_div);
    })
})