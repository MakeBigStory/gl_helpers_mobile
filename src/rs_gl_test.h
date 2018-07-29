//
//  rs_gl_test.h
//  SolarSystem
//
//  Created by michael on 2018/7/28.
//  Copyright © 2018 michaelfx. All rights reserved.
//

#ifndef rs_gl_test_h
#define rs_gl_test_h

//void init_env_rs();

struct GLProgramWrapper {
    void* program;
    float timestamp;
};

struct GLProgramWrapper* rust_opengl_backend_init();
void rust_opengl_backend_draw(struct GLProgramWrapper* wrapper);

#endif /* rs_gl_test_h */
