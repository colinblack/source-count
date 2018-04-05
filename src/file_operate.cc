#include "file_operate.h"
#include "factory.h"


void CFileOperate::readFile(string& path)
{
	DIR *dir;
	struct stat file;
	if (stat(path.c_str(), &file) == -1)
	{
		perror("stat");
		exit(1);
	}

	if (file.st_mode & S_IFDIR)
	{
		readFileList(path);
	}
	else
	{
		fprintf(stderr, "%s, is not a  directory", path.c_str());
		exit(0);
	}
}

int CFileOperate::readFileList(string& path)
{
	DIR *dir;
	struct dirent *ptr = NULL;
	CCountTask* task = NULL;
	string tmp;


	if (path[path.length() - 1] != '/')
	{
		path.push_back('/');
	}

	if ((dir = opendir(path.c_str())) == NULL)
	{
		perror("Open dir error:");
		exit(1);
	}


	while ((ptr = readdir(dir)) != NULL)
	{
		if (strcmp(ptr->d_name, ".") == 0 || strcmp(ptr->d_name, "..") == 0)    
		{
			continue;
		}
		else if (ptr->d_type == 8)     
		{
			task = CFileTypeFactory::getInstance(path + ptr->d_name);
			if (task == NULL)
			{
				continue;
			}
			_pool->AddTask(task); 
		}
		else if (ptr->d_type == 4)     
		{
			tmp = path;
			tmp += ptr->d_name;
			
			readFileList(tmp);
		}
	}
	closedir(dir);
	return 1;
}

