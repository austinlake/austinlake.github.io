const track = document.getElementById("track");

window.onmousedown = e => {
    track.dataset.mouseDownAt = e.clientX;
}


window.onmouseup = () => {
    track.dataset.mouseDownAt = "0";
    track.dataset.prevPercentage = track.dataset.percentage
}

window.onmousemove = e => {
    if(track.dataset.mouseDownAt === "0") return;
    
    const delta = window.onmousemove = parseFloat(track.dataset.mouseDownAt) - e.clientX,
        maxDelta = window.innerWidth/2;
    
    const percentage = (delta / maxDelta) * -100,
        nextPercentage = parseFloat(track.dataset.percentage) + percentage;
    
    track.dataset.percentage = Math.max(Math.min(nextPercentage, 0), -100);
    
    track.animate({
        transform: 'translate(${nextPercentage}%, -50%)'
    }, {duration: 1200, fill: "forwards"});

    for(const image of track.getElementsByClassName("image")) {
        image.animate({
            objectPosition: `${nextPercentage + 100} 50%`
        }, {duration: 1200, fill: "forwards"});
    }
}

