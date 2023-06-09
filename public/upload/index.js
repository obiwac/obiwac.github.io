const upload_form = document.getElementById("form")
const files_input = document.getElementById("files")
const ip_input = document.getElementById("ip")
const button = document.getElementById("button")

upload_form.onsubmit = ((e) => {
	e.preventDefault()

	const files = files_input.files
	const form_data = new FormData()

	for (let file of files)
		form_data.append("files", file)

	const ip = ip_input.value
	const url = `${ip}/api/upload`

	const xhr = new XMLHttpRequest()
	xhr.open("POST", url, true)

	xhr.setRequestHeader("X-Requested-With", "XMLHttpRequest")
	xhr.setRequestHeader("Access-Control-Allow-Origin", "*")

	const orig_button_val = button.value

	xhr.upload.onprogress = (e) => {
		if (!e.lengthComputable)
			return

		const frac = (e.loaded / e.total * 100).toFixed(2)
		button.value = `${frac}%`
		button.disabled = true
	}

	xhr.onload = () => {
		if (xhr.status === 200) {
			button.value = orig_button_val
			button.disabled = false
			console.log(xhr.responseText)
		}

		else
			console.error("upload failed")
	}

	xhr.send(form_data)
})
