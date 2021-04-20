// VAOとVBOを扱う構造体

use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,                          // 頂点データのデータサイズ
        data: *const c_void,                       // 頂点データのポインタ
        usage: GLenum,                             // アクセス頻度を表す定数
        attribute_type_vec: std::vec::Vec<GLenum>, // 各頂点の属性のデータ型のvector
        attribute_size_vec: std::vec::Vec<GLint>,  // 各頂点の属性のデータサイズのvector
        stride: GLsizei,                           // 各頂点データの属性数
        vertex_num: i32,                           // 頂点数
    ) -> Vertex {
        //IDを格納する変数の宣言
        //本体ではなく、IDを格納する
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // GPU上にVAOとVBOを確保する
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // 使用するVAOとVBOを指定する
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            // VAOの設定
            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    attribute_size_vec[i],
                    attribute_type_vec[i],
                    gl::FALSE,
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // VAOとVBOの指定を解除1
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            gl::BindVertexArray(0);
        }
    }
}
