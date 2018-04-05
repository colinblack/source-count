#include "factory.h"
#include "count_task_cpp.h"
#include "count_task_shell.h"

CCountTask* CFileTypeFactory::getInstance(const string& path)
{
	CCountTask *obj = NULL;
	int found = path.find_last_of(".");
	string type = path.substr(found + 1);

	if (type == "c" || type == "cc" || type == "cpp" \
		|| type == "h" || type == "hpp")
	{
		obj = new CCountTaskCPP(path);
	}
	else if (type == ".sh")
	{
		obj = new CCountTaskShell(path);
	}
	else
	{
		//±£¡Ù
	}

	return obj;
}


