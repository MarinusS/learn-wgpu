#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

// lib.rs
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Mesh<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
}

pub const DEMO_TRIANGLE: Mesh = Mesh {
    vertices: &[
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ],
    indices: &[0, 1, 2],
};

pub const DEMO_STRUCTURE: Mesh = Mesh {
    vertices: &[
        Vertex {
            //0
            position: [-0.4, -0.2, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //1
            position: [-0.5, -0.2, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //2
            position: [-0.5, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //3
            position: [-0.4, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //4
            position: [-0.5, 0.22, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //5
            position: [-0.7, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //6
            position: [-0.66, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //7
            position: [-0.5, 0.26, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //8
            position: [-0.7, 0.45, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //9
            position: [0.7, 0.45, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //10
            position: [0.7, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //11
            position: [0.5, -0.2, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //12
            position: [0.4, -0.2, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //13
            position: [0.4, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //14
            position: [0.5, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //15
            position: [0.5, 0.22, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //16
            position: [0.5, 0.26, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //17
            position: [0.66, 0.4, 0.0],
            color: [0.64, 0.176, 0.164],
        },
        Vertex {
            //18
            position: [-0.2, 0.45, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            //19
            position: [0.0, 0.7, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            //20
            position: [0.2, 0.45, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ],
    indices: &[
        1, 0, 3, 1, 3, 2, 4, 7, 5, 7, 6, 5, 5, 9, 8, 5, 10, 9, 12, 14, 13, 12, 11, 14, 15, 10, 16,
        16, 10, 17, 18, 20, 19,
    ],
};
