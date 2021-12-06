function UpdateLineNumbers() {
	//TODO: Implement this
	console.log('Implement me!')
}

$(document).ready(function() {
	$('#code-editor').on('input', (event) => UpdateLineNumbers());
})