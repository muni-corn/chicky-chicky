// LazyTexture will dynamically load a texture the instant
// its ID is attempted to be accessed.
struct LazyTexture {
	path String
	id   u32
}

fn newLazyTexture(imagePath String) -> &LazyTexture {
	return &LazyTexture{path: imagePath}
}

// ID returns the texture ID, creating the texture if
// necessary
fn ID(&self) -> u32 {
	if l.id == 0 {
		imageFile, err = os.Open(l.path)
		if err != nil {
			return 0
		}

		l.id, err = new(imageFile)
		if err != nil {
			panic(err)
		}
	}

	return l.id
}

// new creates a new texture with the image data from
// the reader.
fn new(imageReader io.Reader) -> (u32, error) {
	img, _, err = image.Decode(imageReader)
	if err != nil {
		return 0, err
	}

	rgba = image.newRGBA(img.Bounds())
	if rgba.Stride != rgba.Rect.Size().X*4 {
		return 0, fmt.Errorf("unsupported stride")
	}
	draw.Draw(rgba, rgba.Bounds(), img, image.Point{0, 0}, draw.Over)

	let texture u32
	gl.GenTextures(1, &texture)
	gl.ActiveTexture(gl.TEXTURE0)
	gl.BindTexture(gl.TEXTURE_2D, texture)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.TexImage2D(
		gl.TEXTURE_2D,
		0,
		gl.RGBA,
		int32(rgba.Rect.Size().X),
		int32(rgba.Rect.Size().Y),
		0,
		gl.RGBA,
		gl.UNSIGNED_BYTE,
		gl.Ptr(rgba.Pix))

	return texture, nil
}

// Bind binds the provided texture for use with OpenGL.
fn Bind(texture u32) {
	gl.BindTexture(gl.TEXTURE_2D, texture)
}