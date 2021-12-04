document.addEventListener("DOMContentLoaded", (e) => {
    // const socket = new WebSocket('http://localhost:3030/chat/test?since=1638565521');

    var output = document.getElementById("test-output");
    var input = document.getElementById("test-input");
    
    var postUrl = document.getElementById("post-url");
    var postBody = document.getElementById("post-body");
    var postSend = document.getElementById("post-send");

    postSend.addEventListener("click", (e) => {
        fetch(postUrl.value, {
            method: "POST",
            headers: {'Content-Type': 'application/json'} ,
            body: postBody.value
          }).then(res => {
            console.log("Request complete! response:", res);
          });
    });


    var getUrl = document.getElementById("get-url");
    var getParam = document.getElementById("get-param");
    var getSend = document.getElementById("get-send");
    var getPre = document.getElementById("get-output");

    getSend.addEventListener("click", (e) => {
        fetch(getUrl.value + "?since=" + getParam.value, {
            method: "GET",
            // headers: {'Content-Type': 'application/json'} ,
            // body: postBody.value
          }).then(res => {
            return res.text();
          }).then(text => getPre.innerHTML = text);
    });

    // socket.addEventListener('open', function (event) {
        // input.addEventListener('input', (e) => {
            // socket.send(e.target.value);
        // });
    // });

    // Listen for messages
    // socket.addEventListener('message', function (event) {
        // console.log('Message from server ', event.data);
        // output.textContent = event.data;
    // });

});