#ifndef _FILE_OPERATE_H
#define _FILE_OPERATE_H

#include <iostream>
#include <cstdio>
#include <cstdlib>
#include <string>
#include <queue>
#include <cstring>
#include <sys/types.h>
#include <sys/stat.h>
#include <time.h>
#include <dirent.h>
#include <unistd.h>
#include "thread_pool.h"
using std::string;
using std::queue;
using std::cout;
using std::endl;

class CFileOperate{
public:
	CFileOperate(CThreadPool* p): _pool(p){
	}
	virtual ~CFileOperate(){ delete _pool; }
	 void readFile(string& path);
	 int readFileList(string& path);
public:
	CThreadPool* _pool;
};




#endif
