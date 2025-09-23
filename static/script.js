let selectedFile = null;

function handleDrop(e) {
    e.preventDefault();
    //to check if the first file is a PDF
    if (e.dataTransfer.files.length && e.dataTransfer.files[0].type === "application/pdf") {
        selectedFile = e.dataTransfer.files[0];
        //display the selected file name with ID file-name
        document.getElementById('file-name').textContent = selectedFile.name;
        document.getElementById('summarize-btn').disabled = false;
    }
}

function handleFile(e) {
    //target means when the user selects the file using the file input element
    if (e.target.files.length && e.target.files[0].type === "application/pdf") {
        selectedFile = e.target.files[0];
        document.getElementById('file-name').textContent = selectedFile.name;
        document.getElementById('summarize-btn').disabled = false;
    }
}

async function uploadFile() {
    if (!selectedFile) return;
    //disable the summarize button to prevent multiple clicks
    document.getElementById('summarize-btn').disabled = true;
    document.getElementById('summary').textContent = "Summarizing...";
    // interface,contruct key/value pairs (= fields and values) to send (throguh fetch API) form data(files) to server
    const formData = new FormData();
    //append(name,value,filename(optional))
    formData.append('file',selectedFile); 

    const res = await fetch('/api/summarize', {method: 'POST', body: formData});
    const data = await res.json();
    //If summary is ana array , join it 
    let summaryText = Array.isArray(data.summary) ? data.summary.join('\n') : data.summary;
    //display the summary result(received from backend)
    document.getElementById('summary').textContent = summaryText;

    //Enable download button
    const dl=document.getElementById('download-btn');
    dl.style.display='inline-block';
    //set up download button
    //Blob(["Blob content"], {type:MIME}) give Js temp. files and URL.createObjectURL create a URL for the blob (a collection of binary data stored as a file)
    dl.onclick = () => {
        const blob = new Blob([data.summary], {type: 'text/plain'}); 
        const url=  URL.createObjectURL(blob);
        //create a hidden <a> element,set its href to the file,trigger click to download 
        const a  =document.createElement('a');
        a.href = url;
        a.download = 'summary.txt';
        a.click();
        URL.revokeObjectURL(url); //free up memory
    }
}