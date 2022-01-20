function submit() {
	var key = document.getElementById('encryption').value;
	var text = editor.getSession().getValue();

	if(key != "") {
		var simpleCrypto = new SimpleCrypto(key);
	
		var cipher_text = simpleCrypto.encrypt(text);

		text = cipher_text;
	}

	console.log(text);

	document.getElementById('form-textarea').value = text;
}