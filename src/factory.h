#ifndef   _FACTORY_H_
#define   _FACTORY_H_
#include "count_task.h"

class CFileTypeFactory
{
public:
	static CCountTask* getInstance(const string& path);
};

#endif
