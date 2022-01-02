function UpdateLineNumbers() {
	//TODO: Implement this
	console.log('Implement me!')
}

$(document).ready(function() {
	$('#code-editor').on('input', (event) => UpdateLineNumbers());

	// https://stackoverflow.com/a/6140696
	$("#code-editor").keydown(function(e) {
		console.log("Here");
	
		if(e.keyCode === 9) { // tab was pressed
				// get caret position/selection
				var start = this.selectionStart;
						end = this.selectionEnd;
	
				var $this = $(this);
	
				// set textarea value to: text before caret + tab + text after caret
				$this.val($this.val().substring(0, start)
										+ "\t"
										+ $this.val().substring(end));
	
				// put caret at right position again
				this.selectionStart = this.selectionEnd = start + 1;
	
				// prevent the focus lose
				return false;
		}
	});
	
})