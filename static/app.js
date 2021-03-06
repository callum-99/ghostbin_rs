function encrypt() {
	var key = document.getElementById('encryption').value;
	var text = editor.getSession().getValue();

	if(key != "") {
		var simpleCrypto = new SimpleCrypto(key);
	
		var cipher_text = simpleCrypto.encrypt(text);

		text = cipher_text;
	}

	document.getElementById('form-textarea').value = text;
}

function decrypt(key) {
  if(key == "") {
    document.getElementById('crypto-error').innerHTML = "Empty password!";
    return;
  }

  var cipher_text = editor.getSession().getValue();

  var simpleCrypto = new SimpleCrypto(key);
  try {
    var plain_text = simpleCrypto.decrypt(cipher_text);
    editor.getSession().setValue(plain_text);
    document.getElementById('pass-form').hidden = true;
    document.getElementById('code-editor').hidden = false;
  }
  catch(e) {
    console.error(e);
    document.getElementById('crypto-error').innerHTML = "Incorrect password!";
  }
}

function downloadURI(uri, name) {
  var link = document.createElement("a");
  link.download = name;
  link.href = uri;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  delete link;
}

function load() {
  pasteID = document.getElementById('id').innerText;

  document.getElementById("save").onclick = function() {
    var content = editor.getSession().getValue();
    downloadURI("data:application/txt," + encodeURIComponent(content), pasteID + ".txt");
  }
}

document.addEventListener('DOMContentLoaded', load, false);