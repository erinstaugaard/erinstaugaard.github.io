const canvas = document.getElementById("clock");
const ctx = canvas.getContext("2d");

setInterval(showTime, 1000);
function showTime() {
    let time = new Date();
    let hour = time.getHours();
    let min = time.getMinutes();
    let sec = time.getSeconds();

    const displayWidth = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;

    if (canvas.width !== displayWidth || canvas.height !== displayHeight) {
        canvas.width = displayWidth;
        canvas.height = displayHeight;
    }

    ctx.clearRect(0,0,canvas.width,canvas.height);

    let diameter = Math.min(canvas.width,canvas.height);
    let radius = diameter / 2;

    ctx.strokeStyle = "#cad3f5";
    ctx.lineCap = "round";

    ctx.beginPath();

    ctx.lineWidth = 5;

    let percentEnd = 0.9;
    let percentStart = 0.8;


    for (let i = 0; i < 12; i++) {
        let angle = i/12.0 * Math.PI * 2;

        let x = Math.cos(angle);
        let y = Math.sin(angle);

        ctx.moveTo(radius * x * percentStart + radius, radius * y * percentStart + radius);
        ctx.lineTo(radius * x * percentEnd + radius, radius * y * percentEnd + radius);
    } 

    ctx.stroke();

    ctx.lineCap = "round";

    let hourEnd = 0.5

    {
        ctx.beginPath();
        ctx.moveTo(radius,radius);

        console.log(hour);

        let angle = (hour - 4)/12.0 * Math.PI * 2;

        let x = Math.sin(angle);
        let y = -Math.cos(angle);

        ctx.lineTo(radius * x * hourEnd + radius, radius * y * hourEnd + radius);
        ctx.stroke();
    }

    let minuteEnd = 0.9

    {
        ctx.beginPath();
        ctx.moveTo(radius,radius);

        let angle = min/60.0 * Math.PI * 2;

        let x = Math.sin(angle);
        let y = -Math.cos(angle);

        ctx.lineTo(radius * x * minuteEnd + radius, radius * y * minuteEnd + radius);
        ctx.stroke();
    }

    let secondEnd = 0.9

    {
        ctx.strokeStyle = "#f5bde6";
        ctx.beginPath();
        ctx.moveTo(radius,radius);

        let angle = sec/60.0 * Math.PI * 2;

        let x = Math.sin(angle);
        let y = -Math.cos(angle);

        ctx.lineTo(radius * x * secondEnd + radius, radius * y * secondEnd + radius);
        ctx.stroke();
    }


    console.log("drawn");

    hour =
        hour < 10 ? "0" + hour : hour;
    min = min < 10 ? "0" + min : min;

    let currentTime =
        hour +
        ":" +
        min;

    console.log(currentTime);

    // // Displaying the time
    // document.getElementById(
    //     "clock"
    // ).innerHTML = currentTime;
}

showTime();

let search_input = document.getElementById("search-input");

let search = document.getElementById("search");

search.addEventListener("submit", (event) => {
    navigate_to(`https://duckduckgo.com/?q=${encodeURIComponent(search_input.value)}&ia=web`)
})

function navigate_to(link) {
    console.log(link);
    window.location = link;
    event.preventDefault();
}

document.addEventListener("keydown", function (e) {
    if (e.ctrlKey) {
        switch(e.key) {
          case '1':
            navigate_to("https://github.com/")
            break;
          case '2':
            navigate_to("https://cengage.com")
            break;
          case '3':
            navigate_to("https://verbformen.com")
            break;
          case '4':
            navigate_to("https://dict.cc")
            break;
          case '5':
            navigate_to("https://www.inoreader.com/all_articles")
            break;
          case '6':
            navigate_to("https://moodle.haverford.edu")
            break;
          case '7':
            navigate_to("https://moodle.brynmawr.edu")
            break;
          case '8':
            navigate_to("https://vbm.brynmawr.edu")
            break;
          default:
        }     
    }});
